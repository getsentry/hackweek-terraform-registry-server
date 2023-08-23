ALTER TABLE module 
ADD CONSTRAINT unique_namespace UNIQUE (namespace, name, system, version);