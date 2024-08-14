import pandas as pd
import matplotlib.pyplot as plt
from starlette.applications import Starlette
from starlette.responses import Response, RedirectResponse, StreamingResponse
from starlette.templating import Jinja2Templates
from starlette.routing import Route, Mount
from starlette.staticfiles import StaticFiles
# import uvicorn
import io

DATA_FILE = 'static/game.csv'
templates = Jinja2Templates(directory="templates")

def get_data() -> pd.DataFrame:
    try:
        df = pd.read_csv(DATA_FILE, index_col=0)
    except FileNotFoundError:
        df = pd.DataFrame(columns=['Collin', 'Brennan', 'Ethan', 'Katelyn', 'Mom', 'Dad'])
    return df

async def game_tracker(request):
    df = get_data()
    total_scores = df.sum()
    return templates.TemplateResponse("game_tracker.html", {
        "request": request,
        "game": df.values.tolist(),
        "players": df.columns.tolist(),
        "total_scores": total_scores.tolist()
    })

async def export_csv(_):
    df = get_data()
    data = df.to_csv()
    return Response(data, media_type="text/plain")

async def add_round(request):
    form = await request.form()
    scores = form.getlist('scores')
    df = get_data()
    new_row = pd.DataFrame([scores], columns=df.columns)
    df = pd.concat([df, new_row], ignore_index=True)
    df.to_csv(DATA_FILE)
    return RedirectResponse(url='/', status_code=303)

async def generate_graph(_):
    df = get_data()
    df.iloc[0] = 0
    fig, ax = plt.subplots(figsize=(9, 12))
    df.cumsum().plot(ax=ax)
    
    ax.set_xlabel('Round')
    ax.set_ylabel('Points')
    ax.legend()
    num_rounds = len(df)
    ax.set_xticks(range(0, num_rounds, 1))
    ax.set_xticklabels(range(0, num_rounds, 1))
    
    img = io.BytesIO()
    fig.savefig(img, format='jpg')
    img.seek(0)
    
    return StreamingResponse(img, media_type="image/jpeg")

app = Starlette(debug=False, routes=[
    Route("/", game_tracker),
    Route("/export-csv", export_csv),
    Route("/add-round", add_round, methods=["POST"]),
    Route("/game-graph", generate_graph),
    Mount("/static", StaticFiles(directory="static"), name="static")
])

# if __name__ == "__main__":
#     uvicorn.run(app, host="0.0.0.0", port=8000)
