from celery import Celery
from config import GlobalParams


def make_celery(flask_app):
    params = GlobalParams()
    celery = Celery(
        backend=params.get("QUEUE_BACKEND"),
        broker=params.get("QUEUE_BROKER")
    )
    celery.conf.update(flask_app.config)

    class ContextTask(celery.Task):
        def __call__(self, *args, **kwargs):
            with flask_app.app_context():
                return self.run(*args, **kwargs)

    celery.Task = ContextTask
    return celery
