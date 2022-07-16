import aiohttp
import asyncio
from time import time

async def main():
    start_time: float = time()
    for x in range(100):
        await post_items()
    end_time: float = time()
    print("duration(secs): ", end_time - start_time)        

async def post_items():
    async with aiohttp.ClientSession() as session:
        session.headers.add('Content-Type', 'application/json')
        async with session.post('http://localhost:8000/log',data=b'"mastering rust"') as resp:
            print(resp.status)
            print(await resp.text())

if __name__ == '__main__':
    asyncio.run(main())