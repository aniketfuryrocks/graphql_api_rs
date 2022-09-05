# Rust API template

Bootstrap template for `rust + actix + mongo + jwt` API.

Comes with mongo express for development environment. 

## Open Ports

| name            | port | env        |
| -------------   | ---- | --------   |
| `curiosity-api` | 8080 | `prod/dev` |
| `mongo-express` | 8081 | `dev`      |

## Environment Variables

| env          | when        |
| ---          | ----------- |
| `JWT_SECRET` | PROD        |

## Development

**run using**

```bash
make dev
```

