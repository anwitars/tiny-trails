ALTER TABLE trails
    ADD COLUMN secret VARCHAR(255);

UPDATE trails
    SET secret = '';

ALTER TABLE trails
    ALTER COLUMN secret SET NOT NULL;
