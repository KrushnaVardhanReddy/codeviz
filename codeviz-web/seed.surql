-- Auth.js schema for SurrealDB
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD email ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD emailVerified ON user TYPE option<datetime>;
DEFINE FIELD image ON user TYPE option<string>;
DEFINE INDEX email ON user COLUMNS email UNIQUE;

DEFINE TABLE session SCHEMAFULL;
DEFINE FIELD expires ON session TYPE datetime;
DEFINE FIELD sessionToken ON session TYPE string;
DEFINE FIELD userId ON session TYPE record<user>;
DEFINE INDEX sessionToken ON session COLUMNS sessionToken UNIQUE;

DEFINE TABLE account SCHEMAFULL;
DEFINE FIELD userId ON account TYPE record<user>;
DEFINE FIELD type ON account TYPE string;
DEFINE FIELD provider ON account TYPE string;
DEFINE FIELD providerAccountId ON account TYPE string;
DEFINE INDEX provider_account ON account COLUMNS provider, providerAccountId UNIQUE;

DEFINE TABLE verification_token SCHEMAFULL;
DEFINE FIELD identifier ON verification_token TYPE string;
DEFINE FIELD token ON verification_token TYPE string;
DEFINE FIELD expires ON verification_token TYPE datetime;

-- Test seed data
CREATE user:testuser CONTENT {
  name: "Test User",
  email: "testuser@example.com",
  emailVerified: time::now(),
  image: null
};
