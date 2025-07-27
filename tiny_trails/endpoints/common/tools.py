from hashlib import sha256
from uuid import uuid4

from fastapi import Request


def hash_ip(ip: str) -> str:
    return sha256(ip.encode()).hexdigest()


def get_ip_from_request(request: Request) -> str | None:
    if request.client:
        return request.client.host

    return None


def generate_trail_token() -> str:
    return uuid4().hex
