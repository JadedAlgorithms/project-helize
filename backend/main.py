from fastapi import FastAPI

app = FastAPI(title ="Helize",version="0.1.0")

@app.get("/health")
def health():
    return {"status": "ok"}