FROM python:3.6.13-stretch

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
ADD . /app

RUN apt-get update -y
RUN apt-get install -y python3-dev python-dev gcc

RUN pip install --upgrade pip setuptools wheel
RUN pip install -r requirements.txt

EXPOSE 5002

CMD ["gunicorn", "-w 4", "-b", "0.0.0.0:5002", "app:app"]
