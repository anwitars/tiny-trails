from hashlib import sha256

from fastapi import Request


def hash_ip(ip: str) -> str:
    return sha256(ip.encode()).hexdigest()


def get_ip_from_request(request: Request) -> str | None:
    if request.client:
        return request.client.host

    return None
