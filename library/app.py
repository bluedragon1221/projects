from starlette.applications import Starlette
from starlette.responses import Response, RedirectResponse, StreamingResponse
from starlette.templating import Jinja2Templates
from starlette.routing import Route, Mount
from starlette.staticfiles import StaticFiles
import io

templates = Jinja2Templates(directory="templates")

async def main():
    return "Hello World"

app = Starlette(debug=False, routes=[
    Route("/", main),
    Mount("/static", StaticFiles(directory="static"), name="static")
])
