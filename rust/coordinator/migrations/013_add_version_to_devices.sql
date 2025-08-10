-- Migration: Add version column to devices table
-- This column tracks the software version reported by clients/servers in heartbeats

ALTER TABLE devices ADD COLUMN IF NOT EXISTS version VARCHAR(50);

-- Optionally create an index for querying devices by version
CREATE INDEX IF NOT EXISTS idx_devices_version ON devices(version);