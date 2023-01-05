-- The fantoir table uses French columns to easily identify them
-- from the file description (specification), not translated to English.
-- This table is only to use for France ways, not for other countries,
-- as it's specifically tied up to the FANTOIR database.
--
-- If you provide several instructions, separate those with TWO blank lines.
-- Indexes have to match every WHERE clause used against the database.
--
-- This schema is compiled as part of the program, as such you need to rebuild
-- (`cargo build`) the project after any schema modification.

CREATE TABLE IF NOT EXISTS /*table*/fantoir (
    -- identifiers
    id bigserial
        constraint /*index*/index_fantoir_pk
            primary key,
    code_fantoir char(11) NOT NULL,

    -- Part 1 - commune
    departement varchar(3) NOT NULL,
    code_commune integer NOT NULL,
    code_insee char(5) NOT NULL,
    type_commune varchar(1),
    is_pseudo_recensee bool NOT NULL,

    -- Part 2 - voie
    identifiant_communal_voie varchar(4) NOT NULL,
    cle_rivoli char(1) NOT NULL,
    code_nature_voie varchar(4),
    libelle_voie varchar(26) NOT NULL,
    type_voie smallint NOT NULL,
    is_public bool NOT NULL,

    -- Part 3 - population
    is_large bool NOT NULL,
    population_a_part integer NOT NULL,
    population_fictive integer NOT NULL,

    -- Part 4 - metadata
    is_cancelled bool NOT NULL,
    cancel_date DATE,
    creation_date DATE,
    code_majic integer NOT NULL,
    last_alpha_word varchar(8) NOT NULL,

    -- Constraints
    UNIQUE (code_fantoir),
    UNIQUE (code_insee, identifiant_communal_voie)
);


CREATE INDEX CONCURRENTLY /*index*/index_fantoir_voie_trigram
    ON /*table*/fantoir
        USING gin (libelle_voie gin_trgm_ops);
