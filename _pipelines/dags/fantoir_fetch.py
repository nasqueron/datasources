#   -------------------------------------------------------------
#   Nasqueron Datasources :: pipelines
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#   Project:        Nasqueron
#   Pipeline:       Datasources > FANTOIR > fetch
#   License:        BSD-2-Clause
#   -------------------------------------------------------------

from datetime import datetime
import json
import requests

from airflow.decorators import dag, task
from airflow.models import Variable, TaskInstance
from airflow.operators.python import ShortCircuitOperator
from airflow.operators.trigger_dagrun import TriggerDagRunOperator

from nasqueron_datasources.pipelines.commands import run, parse_environment
from nasqueron_datasources.pipelines.errors import CommandException, WorkflowException


NOTIFICATION_URL = "https://notifications.nasqueron.org/gate/Notification/Nasqueron"


@dag(
    dag_id="fantoir_fetch",
    schedule=None,
    start_date=datetime(2023, 1, 1),
    tags=["datasources", "fantoir", "download", "external"],
)
def fantoir_fetch_dag():
    """
    ### Pipeline for FANTOIR datasource - fetch

    This pipeline checks if a new version of FANTOIR file is published.

    If so it downloads it, extracts it and calls import DAG.

    Reference: https://agora.nasqueron.org/Fantoir-datasource
    """

    @task
    def fetch() -> dict:
        """Fetches FANTOIR from data.economie.gouv.fr, if a new version is available."""
        exit_code, stdout, stderr = run(
            ["fantoir-datasource", "fetch"],
            cwd=Variable.get("fantoir_directory"),
            env={
                "DATABASE_URL": "",  # a value is unneeded for fetch operation
            },
        )

        if exit_code == 12:
            # No new version available
            return {
                "new_version": False,
                "environment": {},
            }

        if exit_code != 0:
            # Failure
            raise CommandException("Can't fetch FANTOIR", exit_code, stderr)

        return {
            "new_version": True,
            "environment": parse_environment(stdout),
        }

    def is_new_version_available(task_instance: TaskInstance) -> bool:
        return task_instance.xcom_pull(task_ids="fetch", key="new_version")

    check_fetch = ShortCircuitOperator(
        task_id="check_fetch",
        python_callable=is_new_version_available,
        doc_md="""Determine if a new version is available from previous task.""",
    )

    # Triggered by fantoir_fetch DAG, as a new version is available.
    call_import_dag = TriggerDagRunOperator(
        task_id="call_import_dag",
        trigger_dag_id="fantoir_import",
        conf={
            "fantoir_environment": "{{ task_instance.xcom_pull(task_ids='fetch', key='environment') }}"
        },
        doc_md="""Launch the workflow to import FANTOIR new version""",
    )

    @task
    def notify(task_instance: TaskInstance):
        """Sends a notification a new version is available."""

        fantoir_file = task_instance.xcom_pull(task_ids="fetch", key="environment").get(
            "FANTOIR_FILE", "(unknown)"
        )
        dag_run_id = task_instance.xcom_pull(
            task_ids="call_import_dag", key="trigger_run_id"
        )
        notification = {
            "service": "Airflow",
            "project": "Nasqueron",
            "group": "Datasources",
            "type": "fantoir-fetch",
            "text": f"A new version of FANTOIR has been fetched: {fantoir_file}. Triggering import workflow: {dag_run_id}.",
        }

        response = requests.post(NOTIFICATION_URL, data=json.dumps(notification))
        if response.status_code != 200:
            raise WorkflowException(
                "Can't send notification: HTTP error " + str(response.status_code)
            )

    fetch() >> check_fetch >> call_import_dag >> notify()


fantoir_fetch_dag()
