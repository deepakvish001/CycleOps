CREATE INDEX pickup_requests_tenant_status_requested_idx ON pickup_requests (tenant_id, status, requested_at);
CREATE INDEX pickup_requests_tenant_location_gist ON pickup_requests USING gist (location) WHERE status IN ('requested','scheduled');
CREATE INDEX service_zones_tenant_name_idx ON service_zones (tenant_id, name);
CREATE INDEX route_plans_tenant_service_date_idx ON route_plans (tenant_id, service_date DESC);
