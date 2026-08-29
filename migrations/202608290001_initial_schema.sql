CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE tenants (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), name text NOT NULL, created_at timestamptz NOT NULL DEFAULT now());
CREATE TABLE service_zones (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), tenant_id uuid NOT NULL REFERENCES tenants(id), name text NOT NULL, boundary geography(MultiPolygon, 4326) NOT NULL);
CREATE INDEX service_zones_boundary_gist ON service_zones USING gist(boundary);
CREATE TABLE pickup_requests (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), tenant_id uuid NOT NULL REFERENCES tenants(id), zone_id uuid NOT NULL REFERENCES service_zones(id), reference text NOT NULL, status text NOT NULL DEFAULT 'requested', location geography(Point, 4326) NOT NULL, requested_at timestamptz NOT NULL DEFAULT now(), UNIQUE (tenant_id, reference));
CREATE INDEX pickup_requests_location_gist ON pickup_requests USING gist(location);
CREATE TABLE route_plans (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), tenant_id uuid NOT NULL REFERENCES tenants(id), service_date date NOT NULL, status text NOT NULL DEFAULT 'draft');
CREATE TABLE weighbridge_tickets (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), tenant_id uuid NOT NULL REFERENCES tenants(id), gross_kg numeric(14,3) NOT NULL, tare_kg numeric(14,3) NOT NULL, recorded_at timestamptz NOT NULL, CHECK (gross_kg >= tare_kg));
CREATE TABLE recovery_batches (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), tenant_id uuid NOT NULL REFERENCES tenants(id), input_kg numeric(14,3) NOT NULL, recovered_kg numeric(14,3), status text NOT NULL DEFAULT 'open');
CREATE TABLE settlements (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), tenant_id uuid NOT NULL REFERENCES tenants(id), amount_minor bigint NOT NULL, currency char(3) NOT NULL, status text NOT NULL DEFAULT 'pending');
