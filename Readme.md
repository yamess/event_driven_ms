# Event Driven Microservice

This is a simple example of an event driven microservice using Redis Stream as the message broker and actor model for processing the messages in parallele.

## Dependencies

### Environment variables
    HOST: Application host. default: localhost
    PORT: Application port. default: 3000
    REDIS_CONNECTION_STRING: Redis host. default: localhost

### How to run

```bash
make run
```
