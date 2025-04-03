import asyncio
import target.debug.iroh_uniffi as iroh_uniffi
from target.debug.iroh_uniffi import Endpoint, IrohError


async def accept(endpoint: Endpoint):
    try:
        conn = await endpoint.accept()
        print(f'Accepted connection from {conn.node_id_short()}')

        bi_stream = await conn.accept_bi()
        recv = bi_stream.recv_stream()

        while (chunk := await recv.read_chunk(20, True)) is not None:
            if chunk:
                print(f'Received chunk {chunk.bytes()}')
    except IrohError as err:
        print(err.message())
        raise err


async def connect_to(endpoint: Endpoint, other_id: str):
    try:
        conn = await endpoint.connect(other_id, b'dummy')
        print(f'Connected to {conn.node_id_short()}')

        bi_stream = await conn.open_bi()
        send = bi_stream.send_stream()

        for i in range(10):
            await send.write(f'Message {i}\0'.encode())
        await send.finish()
        await send.stopped()
        print('This is the end')
    except IrohError as err:
        print(err.message())
        raise err


async def main():
    endpoint_1 = await iroh_uniffi.new_endpoint_builder().discovery_n0().alpns([b'dummy']).bind()
    endpoint_2 = await iroh_uniffi.new_endpoint_builder().discovery_n0().alpns([b'dummy']).bind()
    await asyncio.sleep(2)
    await asyncio.gather(accept(endpoint_1),
                         connect_to(endpoint_2, endpoint_1.node_id()))

if __name__ == '__main__':
    asyncio.run(main())
