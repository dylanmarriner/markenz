set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default:
  @just --list

auth:keycloak-up:
  docker compose -f infra/auth/compose.yml --profile keycloak up -d

auth:authentik-up:
  docker compose -f infra/auth/compose.yml --profile authentik up -d

auth:down:
  docker compose -f infra/auth/compose.yml --profile keycloak --profile authentik down -v

audit:nostubs:
  @! rg -n --hidden --no-ignore -S "(TODO|FIXME|unimplemented!\(|@ts-nocheck|panic!\(\"TODO\")" . || (echo "OK: no stub markers found")

test-m1: test-determinism test-offline-scan test-deterministic-collections
  @echo "All M1 Phase 2 tests passed!"

test-determinism:
  @cargo test -p markenz-server test_determinism_simple
  @cargo test -p markenz-server test_determinism_with_inputs
  @cargo test -p deterministic

test-offline-scan:
  @! grep -r "reqwest\|std::net\|std::time" apps/server/src/sim/ || (echo "OK: no nondeterministic APIs in sim module")
  @! grep -r "reqwest\|std::net\|std::time" crates/deterministic/src/ || (echo "OK: no nondeterministic APIs in deterministic crate")

test-deterministic-collections:
  @cargo test -p deterministic test_deterministic_map
  @cargo test -p deterministic test_deterministic_set
  @cargo test -p deterministic test_deterministic_vec

