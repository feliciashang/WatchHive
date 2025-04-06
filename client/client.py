import typer
import websockets
import uvloop
from typing import Annotated
import time

app = typer.Typer()


async def connect_websocket(uri: str):
  async with websockets.connect(uri) as websocket:
    typer.echo("Connected to WebSocket server")

    while True:
      try:
        # Send a message
        message = input("> ")
        start = time.time_ns()
        await websocket.send(message)

        # Receive response
        response = await websocket.recv()
        end = time.time_ns()
        time_ms = (end - start) / 1000000
        typer.echo(f"Received: {response} in {time_ms}ms")
      except websockets.exceptions.ConnectionClosed:
        typer.echo("Connection closed")
        break
      except KeyboardInterrupt:
        typer.echo("\nExiting...")
        break


@app.command()
def main(
  host: Annotated[str, typer.Option(help="WebSocket server host")] = "127.0.0.1",
  port: Annotated[int, typer.Option(help="WebSocket server port")] = 3000,
):
  """Simple WebSocket client for testing"""
  uri = f"ws://{host}:{port}/ws"
  typer.echo(f"Connecting to {uri}")
  uvloop.run(connect_websocket(uri))


if __name__ == "__main__":
  app()
