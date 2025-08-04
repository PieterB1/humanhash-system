import ray
from ray import serve
import logging
import time
import sys

logging.basicConfig(level=logging.DEBUG, force=True, stream=sys.stderr)
logger = logging.getLogger(__name__)
handler = logging.StreamHandler(sys.stderr)
handler.setLevel(logging.DEBUG)
logger.addHandler(handler)
logger.setLevel(logging.DEBUG)

try:
    logger.debug("Waiting 30 seconds for GCS to be ready")
    time.sleep(30)
    logger.debug("Initializing Ray")
    ray.init(address="127.0.0.1:6379", ignore_reinit_error=True)
    logger.debug("Ray initialized")

    @serve.deployment(route_prefix="/")
    def hello():
        logger.debug("Handling request to root endpoint")
        return "Hello from Ray microservice!"

    logger.debug("Starting Serve")
    serve.start(http_options={"host": "0.0.0.0", "port": 8000}, detached=True)
    logger.debug("Serve started")

    logger.debug("Deploying Serve app")
    app = hello.bind()
    serve.run(app)
    logger.debug("Serve app deployed")

    while True:
        time.sleep(1)
except Exception as e:
    logger.error(f"Error in script: {str(e)}")
    raise

