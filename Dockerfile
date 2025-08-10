FROM python:3.12-alpine as builder

ENV PYTHONUNBUFFERED=1 \
	POETRY_VIRTUALENVS_IN_PROJECT=false \
	POETRY_HOME=/poetry

WORKDIR /build

RUN apk add --no-cache \
	postgresql-dev \
	gcc \
	musl-dev \
	python3-dev

RUN python -m pip install --upgrade pip
RUN pip install poetry==2.1.3

COPY pyproject.toml poetry.lock LICENSE README.md ./
COPY tiny_trails/ tiny_trails/

RUN poetry build -f wheel
RUN pip wheel --no-deps dist/*.whl -w /wheels && \
	pip wheel . -w /wheels

FROM python:3.12-alpine as runtime


RUN apk add --no-cache postgresql-libs

COPY --from=builder /wheels /wheels
RUN pip install --no-index --find-links=/wheels tiny-trails

CMD ["tiny-trails", "serve", "--host", "0.0.0.0"]
