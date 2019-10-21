import graphene
from fastapi import FastAPI
from starlette.graphql import GraphQLApp


class Query(graphene.ObjectType):
    test = graphene.Float(a=graphene.Float(), b=graphene.Float())
    hello = graphene.String(name=graphene.String(default_value="Anonymous"))

    def resolve_test(self, info, a, b):
        return a + b
    
    def resolve_hello(self, info, name):
        return f"Hello "


app = FastAPI()
app.add_route("/graphql", GraphQLApp(schema=graphene.Schema(query=Query)))
