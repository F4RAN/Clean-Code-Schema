import asyncio
from functools import wraps

def sync_to_async(func):
    """Decorator to wrap synchronous PyMongo operations in asyncio.to_thread"""
    @wraps(func)
    async def wrapper(*args, **kwargs):
        # Execute the sync function in a thread pool
        return await asyncio.to_thread(func, *args, **kwargs)
    return wrapper