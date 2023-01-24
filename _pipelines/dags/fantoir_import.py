#   -------------------------------------------------------------
#   Nasqueron Datasources :: pipelines
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#   Project:        Nasqueron
#   Pipeline:       Datasources > FANTOIR > import
#   License:        BSD-2-Clause
#   -------------------------------------------------------------

from datetime import datetime

from airflow.decorators import dag, task
from airflow.models import Connection, Variable

from nasqueron_datasources.pipelines.commands import run


@dag(
    dag_id="fantoir_import",
    schedule=None,
    start_date=datetime(2023, 1, 1),
    tags=["datasources", "fantoir", "postgresql", "external"],
)
def fantoir_import_dag():
    """
    ### Pipeline for FANTOIR datasource - import

    This pipeline imports FANTOIR into PostgreSQL, enriches it
    and promotes the new table as the one to use.

    Enrichment is done by fetching information from:
      - Wikidata

    Reference: https://agora.nasqueron.org/Fantoir-datasource
    """

    fantoir_directory = Variable.get("fantoir_directory")
    database_url = Connection.get_connection_from_secrets("postgresql_fantoir").get_uri()

    @task
    def import_to_pgsql():
        run(
            [
                "fantoir-datasource",
                "import",
                "{{ params['FANTOIR_FILE'] }}",
                "{{ params['FANTOIR_TABLE'] }}",
                "-c",
            ],
            cwd=fantoir_directory,
            env={
                "DATABASE_URL": database_url,
            },
        )

    @task
    def enrich_from_wikidata():
        run(
            ["fantoir-datasource", "wikidata"],
            cwd=fantoir_directory,
            env={
                "DATABASE_URL": database_url,
            },
        )

    @task
    def promote():
        run(
            ["fantoir-datasource", "promote"],
            cwd=fantoir_directory,
            env={
                "DATABASE_URL": database_url,
            },
        )

    @task
    def publish_to_configuration():
        """
        NOT IMPLEMENTED.

        Publish new table name to use to etcd/consul
        """
        pass

    @task
    def notify():
        """
        NOT IMPLEMENTED.

        Send notification payload to Notifications Center
        """
        pass

    (
        import_to_pgsql()
        >> [
            # Enrichment sources can run in //.
            enrich_from_wikidata(),
        ]
        >> promote()
        >> [
            # Post-action tasks can run in // too.
            publish_to_configuration(),
            notify(),
        ]
    )


fantoir_import_dag()
