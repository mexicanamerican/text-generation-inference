## Getting Started with Docker

The easiest way of getting started is using the official Docker container. Install Docker following [their installation instructions](https://docs.docker.com/get-docker/).

## Deploying a Model with TGI

Let's say you want to deploy the [Falcon-7B Instruct](https://huggingface.co/tiiuae/falcon-7b-instruct) model with TGI. Here is an example of how to do that:

```bash
model=tiiuae/falcon-7b-instruct
volume=$PWD/data # share a volume with the docker container to avoid downloading weights every run
docker run --gpus all --shm-size 1g -p 8080:80 -v $volume:/data ghcr.io/huggingface/text-generation-inference:1.0.0 --model-id $model
```

The easiest way of getting started is using the official Docker container. Install Docker following [their installation instructions](https://docs.docker.com/get-docker/).

Let's say you want to deploy [Falcon-7B Instruct](https://huggingface.co/tiiuae/falcon-7b-instruct) model with TGI. Here is an example on how to do that:

```bash
model=tiiuae/falcon-7b-instruct
volume=$PWD/data # share a volume with the Docker container to avoid downloading weights every run

docker run --gpus all --shm-size 1g -p 8080:80 -v $volume:/data ghcr.io/huggingface/text-generation-inference:1.0.0 --model-id $model
```

<Tip warning={true}>

## Using GPUs and NVIDIA Container Toolkit
 
To use GPUs with TGI, you need to install the [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/install-guide.html). We also recommend using NVIDIA drivers with CUDA version 11.8 or higher.

## Querying the Endpoints

Once TGI is running, you can query the `generate` endpoint by making requests. To learn how to do this, check the [Consuming TGI](./basic_tutorials/consuming_tgi) section.

```bash
curl 127.0.0.1:8080/generate -X POST -d '{"inputs":"What is Deep Learning?","parameters":{"max_new_tokens":20}}' -H 'Content-Type: application/json'
```

</Tip>

Once TGI is running, you can use the `generate` endpoint by doing requests. To learn more about how to query the endpoints, check the [Consuming TGI](./basic_tutorials/consuming_tgi) section.

```bash
curl 127.0.0.1:8080/generate -X POST -d '{"inputs":"What is Deep Learning?","parameters":{"max_new_tokens":20}}' -H 'Content-Type: application/json'
```

<Tip>

To see all possible flags and options, you can use the `--help` flag. It's possible to configure the number of shards, quantization, generation parameters, and more.

```bash
docker run ghcr.io/huggingface/text-generation-inference:1.0.0 --help
```

</Tip>
