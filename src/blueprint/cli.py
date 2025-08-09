import typer

app = typer.Typer(help="Your CLI — interactive and extensible")

@app.command()
def hello(name: str = "world"):
    """Example command."""
    typer.echo(f"Hello {name}!")

def main():
    app()