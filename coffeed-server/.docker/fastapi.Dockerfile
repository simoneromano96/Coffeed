FROM tiangolo/uvicorn-gunicorn-fastapi:python3.7-alpine3.8

# Install dependencies
RUN pip install \ 
    # GraphQL
    graphene \
    # Pydantic (python types)
    pydantic

# DEV ONLY!
CMD [ "/start-reload.sh" ]
