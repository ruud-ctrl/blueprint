python3 -m venv .venv
source .venv/bin/activate

mkdir -p src/blueprint/{commands,domain,utils,plugins} tests config docs scripts
touch src/blueprint/{__init__.py,__main__.py,cli.py,colors.py,prompts.py,settings.py}
touch src/blueprint/commands/{__init__.py,init_project.py,select_runtime.py,pick_features.py}
touch src/blueprint/domain/{__init__.py,services.py}
touch src/blueprint/utils/{__init__.py,fs.py}
touch README.md LICENSE .gitignore pyproject.toml config/logging.yaml

pip install typer rich InquirerPy pydantic pydantic-settings pyyaml
pip install pytest ruff black mypy pre-commit  # dev tools

