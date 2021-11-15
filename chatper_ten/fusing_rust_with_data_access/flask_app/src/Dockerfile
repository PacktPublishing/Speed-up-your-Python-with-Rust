FROM python:3.6.13-stretch

# Set the working directory to /app
WORKDIR /app

RUN apt-get update -y
RUN apt-get install -y python3-dev python-dev gcc

# setup rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy the current directory contents into the container at /app
ADD . /app

RUN rm ./config.yml
RUN mv live_config.yml config.yml

RUN apt-get update -y
RUN apt-get install -y python3-dev python-dev gcc

RUN pip install --upgrade pip setuptools wheel
RUN pip install -r requirements.txt

# Install the dependencies
RUN pip install --upgrade pip setuptools wheel
RUN pip install -r requirements.txt
# RUN pip install -r git_repos.txt

RUN pip install ./flitton-fib-rs
RUN rm -rf ./flitton-fib-rs

EXPOSE 5002

CMD ["gunicorn", "-w 4", "-b", "0.0.0.0:5002", "app:app"]
