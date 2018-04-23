CREATE TABLE locales (
  id BIGSERIAL PRIMARY KEY,
  lang VARCHAR(8) NOT NULL,
  code VARCHAR(255) NOT NULL,
  message TEXT NOT NULL,
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX idx_locales_code_lang ON locales (code, lang);
CREATE INDEX idx_locales_code ON locales (code);
CREATE INDEX idx_locales_lang ON locales (lang);
