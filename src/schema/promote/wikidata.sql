ALTER TABLE fantoir_wikidata
    DROP CONSTRAINT IF EXISTS fantoir_wikidata_code_fantoir_fk;


ALTER TABLE fantoir_wikidata
    ADD CONSTRAINT fantoir_wikidata_code_fantoir_fk
        FOREIGN KEY (code_fantoir) REFERENCES /*table*/fantoir (code_fantoir);
