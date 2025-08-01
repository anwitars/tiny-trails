FROM python:3.12-alpine as builder

ENV PYTHONUNBUFFERED=1 \
	POETRY_VIRTUALENVS_IN_PROJECT=false \
	POETRY_HOME=/poetry

WORKDIR /build

RUN apk add --no-cache \
	postgresql-dev

RUN python -m pip install --upgrade pip
RUN pip install poetry==2.1.3

COPY pyproject.toml poetry.lock alembic.ini LICENSE README.md ./
COPY tiny_trails/ tiny_trails/

RUN poetry build -f wheel

FROM python:3.12-alpine as runtime

COPY --from=builder /build/dist/*.whl /tmp/

RUN python -m pip install --upgrade pip
RUN python -m pip install /tmp/*.whl

CMD ["tiny-trails", "serve", "--host", "0.0.0.0"]
