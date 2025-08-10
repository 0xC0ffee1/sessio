-- Database Schema for Sessio Coordinator

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- ==============================================================================
-- ACCOUNTS TABLE
-- ==============================================================================
CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    passkey_public_key BYTEA,
    passkey_credential_id TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- ==============================================================================
-- INSTALL KEYS TABLE
-- ==============================================================================
CREATE TABLE IF NOT EXISTS install_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    install_key VARCHAR(64) UNIQUE NOT NULL,
    device_id VARCHAR(255),
    category_name VARCHAR(255),
    categories JSONB DEFAULT '[]'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    used_at TIMESTAMP WITH TIME ZONE,
    expires_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() + INTERVAL '7 days'
);

-- ==============================================================================
-- CATEGORIES TABLE
-- ==============================================================================
CREATE TABLE IF NOT EXISTS categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(account_id, name)
);

-- ==============================================================================
-- DEVICES TABLE
-- ==============================================================================
CREATE TABLE IF NOT EXISTS devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    device_id VARCHAR(255) NOT NULL,
    os_name VARCHAR(100),
    public_key TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_seen_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    -- Device signing fields
    signature TEXT,
    signed_at TIMESTAMPTZ,
    signer_credential_id TEXT,
    -- JWT token tracking
    jwt_token_issued_at TIMESTAMPTZ,
    -- Version tracking
    version VARCHAR(50),
    -- Unique constraint per account
    CONSTRAINT devices_device_id_account_unique UNIQUE (device_id, account_id)
);

-- ==============================================================================
-- DEVICE CATEGORIES JUNCTION TABLE
-- ==============================================================================
CREATE TABLE IF NOT EXISTS device_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    category_id UUID NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Ensure a device can't be assigned to the same category twice
    UNIQUE(device_id, category_id)
);

-- ==============================================================================
-- SESSIONS TABLE
-- ==============================================================================
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    server_device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    client_device_id UUID REFERENCES devices(id) ON DELETE CASCADE,
    device_public_key TEXT,
    ipv6 BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    status VARCHAR(50) DEFAULT 'active'
);

-- ==============================================================================
-- AUTHENTICATION CHALLENGES TABLE
-- ==============================================================================
CREATE TABLE IF NOT EXISTS auth_challenges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    challenge_data TEXT NOT NULL,
    device_public_key TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() + INTERVAL '5 minutes',
    used BOOLEAN DEFAULT FALSE
);

-- ==============================================================================
-- WEBAUTHN CREDENTIALS TABLE
-- ==============================================================================
CREATE TABLE IF NOT EXISTS webauthn_credentials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    credential_id TEXT NOT NULL UNIQUE,
    public_key BYTEA NOT NULL,
    counter INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_used_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    user_handle BYTEA,
    backup_eligible BOOLEAN DEFAULT false,
    backup_state BOOLEAN DEFAULT false,
    attestation_type TEXT,
    user_verified BOOLEAN DEFAULT false
);

-- ==============================================================================
-- WEBAUTHN REGISTRATION SESSIONS TABLE
-- ==============================================================================
CREATE TABLE IF NOT EXISTS webauthn_registration_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    session_data TEXT NOT NULL, -- JSON serialized challenge data
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() + INTERVAL '15 minutes'
);

-- ==============================================================================
-- WEBAUTHN AUTHENTICATION SESSIONS TABLE
-- ==============================================================================
CREATE TABLE IF NOT EXISTS webauthn_authentication_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_data TEXT NOT NULL, -- JSON serialized challenge data  
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() + INTERVAL '15 minutes'
);

-- ==============================================================================
-- USER SESSIONS TABLE
-- ==============================================================================
CREATE TABLE IF NOT EXISTS user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    session_token VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    user_agent TEXT,
    ip_address TEXT
);

-- ==============================================================================
-- BACKWARD COMPATIBILITY - Add missing columns for upgrades
-- ==============================================================================

-- Add columns that may be missing from older schemas
ALTER TABLE accounts ADD COLUMN IF NOT EXISTS passkey_public_key BYTEA;
ALTER TABLE accounts ADD COLUMN IF NOT EXISTS passkey_credential_id TEXT;

ALTER TABLE install_keys ADD COLUMN IF NOT EXISTS device_id VARCHAR(255);
ALTER TABLE install_keys ADD COLUMN IF NOT EXISTS category_name VARCHAR(255);
ALTER TABLE install_keys ADD COLUMN IF NOT EXISTS categories JSONB DEFAULT '[]'::jsonb;

ALTER TABLE devices ADD COLUMN IF NOT EXISTS public_key TEXT;
ALTER TABLE devices ADD COLUMN IF NOT EXISTS signature TEXT;
ALTER TABLE devices ADD COLUMN IF NOT EXISTS signed_at TIMESTAMPTZ;
ALTER TABLE devices ADD COLUMN IF NOT EXISTS signer_credential_id TEXT;
ALTER TABLE devices ADD COLUMN IF NOT EXISTS jwt_token_issued_at TIMESTAMPTZ;
ALTER TABLE devices ADD COLUMN IF NOT EXISTS version VARCHAR(50);

ALTER TABLE sessions ADD COLUMN IF NOT EXISTS device_public_key TEXT;

-- Add unique constraint if it doesn't exist
DO $$ 
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'devices_device_id_account_unique'
    ) THEN
        ALTER TABLE devices ADD CONSTRAINT devices_device_id_account_unique UNIQUE (device_id, account_id);
    END IF;
END $$;

-- ==============================================================================
-- INDEXES
-- ==============================================================================

-- Install Keys indexes
CREATE INDEX IF NOT EXISTS idx_install_keys_account_id ON install_keys(account_id);
CREATE INDEX IF NOT EXISTS idx_install_keys_expires_at ON install_keys(expires_at);
CREATE INDEX IF NOT EXISTS idx_install_keys_device_id ON install_keys(device_id);
CREATE INDEX IF NOT EXISTS idx_install_keys_category_name ON install_keys(category_name);
CREATE INDEX IF NOT EXISTS idx_install_keys_categories ON install_keys USING GIN (categories);

-- Categories indexes
CREATE INDEX IF NOT EXISTS idx_categories_account_id ON categories(account_id);

-- Devices indexes
CREATE INDEX IF NOT EXISTS idx_devices_account_id ON devices(account_id);
CREATE INDEX IF NOT EXISTS idx_devices_signer_credential ON devices(signer_credential_id) WHERE signer_credential_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_devices_version ON devices(version);

-- Device Categories indexes
CREATE INDEX IF NOT EXISTS idx_device_categories_device_id ON device_categories(device_id);
CREATE INDEX IF NOT EXISTS idx_device_categories_category_id ON device_categories(category_id);

-- Sessions indexes
CREATE INDEX IF NOT EXISTS idx_sessions_server_device_id ON sessions(server_device_id);
CREATE INDEX IF NOT EXISTS idx_sessions_client_device_id ON sessions(client_device_id);
CREATE INDEX IF NOT EXISTS idx_sessions_status ON sessions(status);
CREATE INDEX IF NOT EXISTS idx_sessions_device_public_key ON sessions(device_public_key);

-- Auth Challenges indexes
CREATE INDEX IF NOT EXISTS idx_auth_challenges_expires_at ON auth_challenges(expires_at);
CREATE INDEX IF NOT EXISTS idx_auth_challenges_device_public_key ON auth_challenges(device_public_key);

-- WebAuthn Credentials indexes
CREATE INDEX IF NOT EXISTS idx_webauthn_credentials_account_id ON webauthn_credentials(account_id);
CREATE INDEX IF NOT EXISTS idx_webauthn_credentials_credential_id ON webauthn_credentials(credential_id);

-- WebAuthn Registration Sessions indexes
CREATE INDEX IF NOT EXISTS idx_webauthn_registration_sessions_account_id ON webauthn_registration_sessions(account_id);
CREATE INDEX IF NOT EXISTS idx_webauthn_registration_sessions_expires_at ON webauthn_registration_sessions(expires_at);

-- WebAuthn Authentication Sessions indexes
CREATE INDEX IF NOT EXISTS idx_webauthn_authentication_sessions_expires_at ON webauthn_authentication_sessions(expires_at);

-- User Sessions indexes
CREATE INDEX IF NOT EXISTS idx_user_sessions_token ON user_sessions(session_token);
CREATE INDEX IF NOT EXISTS idx_user_sessions_account_id ON user_sessions(account_id);
CREATE INDEX IF NOT EXISTS idx_user_sessions_expires_at ON user_sessions(expires_at);

-- ==============================================================================
-- TRIGGERS
-- ==============================================================================

-- Accounts triggers
DROP TRIGGER IF EXISTS update_accounts_updated_at ON accounts;
CREATE TRIGGER update_accounts_updated_at BEFORE UPDATE ON accounts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Categories triggers
DROP TRIGGER IF EXISTS update_categories_updated_at ON categories;
CREATE TRIGGER update_categories_updated_at BEFORE UPDATE ON categories
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Devices triggers
DROP TRIGGER IF EXISTS update_devices_updated_at ON devices;
CREATE TRIGGER update_devices_updated_at BEFORE UPDATE ON devices
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Sessions triggers
DROP TRIGGER IF EXISTS update_sessions_updated_at ON sessions;
CREATE TRIGGER update_sessions_updated_at BEFORE UPDATE ON sessions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();