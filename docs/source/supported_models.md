# Supported Models and Hardware

Text Generation Inference enables serving optimized models on specific hardware for the highest performance. The following sections list which models are hardware are supported.

## Supported Models

The following models are optimized and can be served with TGI, which uses custom CUDA kernels for better inference. You can add the flag `--disable-custom-kernels` at the end of the `docker run` command if you wish to disable them.

- [BLOOM](https://huggingface.co/bigscience/bloom)
- [FLAN-T5](https://huggingface.co/google/flan-t5-xxl)
- [Galactica](https://huggingface.co/facebook/galactica-120b)
- [GPT-Neox](https://huggingface.co/EleutherAI/gpt-neox-20b)
- [Llama](https://github.com/facebookresearch/llama)
- [OPT](https://huggingface.co/facebook/opt-66b)
- [SantaCoder](https://huggingface.co/bigcode/santacoder)
- [Starcoder](https://huggingface.co/bigcode/starcoder)
- [Falcon 7B](https://huggingface.co/tiiuae/falcon-7b)
- [Falcon 40B](https://huggingface.co/tiiuae/falcon-40b)
- [MPT](https://huggingface.co/mosaicml/mpt-30b)
- [Llama V2](https://huggingface.co/meta-llama)
- [New Model 1](https://huggingface.co/newmodel1)
- [New Model 2](https://huggingface.co/newmodel2)
- [New Model 3](https://huggingface.co/newmodel3)
- [New Model 4](https://huggingface.co/newmodel4)
- [New Model 5](https://huggingface.co/newmodel5)
- [New Model 6](https://huggingface.co/newmodel6)
- [New Model 7](https://huggingface.co/newmodel7)
- [New Model 8](https://huggingface.co/newmodel8)
- [New Model 9](https://huggingface.co/newmodel9)
- [New Model 10](https://huggingface.co/newmodel10)

If the above list lacks the model you would like to serve, depending on the model's pipeline type, you can try to initialize and serve the model anyways to see how well it performs, but performance isn't guaranteed for non-optimized models:

```python
# for causal LMs/text-generation models
AutoModelForCausalLM.from_pretrained(<model>, device_map="auto")`
# or, for text-to-text generation models
AutoModelForSeq2SeqLM.from_pretrained(<model>, device_map="auto")
```


## Supported Hardware

TGI optimized models are supported on NVIDIA [A100](https://www.nvidia.com/en-us/data-center/a100/), [A10G](https://www.nvidia.com/en-us/data-center/products/a10-gpu/) and [T4](https://www.nvidia.com/en-us/data-center/tesla-t4/) GPUs with CUDA 11.8+. Note that you have to install [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/install-guide.html) to use it. For other hardware, continuous batching will still apply, but some operations like flash attention and paged attention will not be executed. 

TGI is also supported on the following AI hardware accelerators:
- *Habana first-gen Gaudi and Gaudi2:* check out this [example](https://github.com/huggingface/optimum-habana/tree/main/text-generation-inference) how to serve models with TGI on Gaudi and Gaudi2 with [Optimum Habana](https://huggingface.co/docs/optimum/habana/index)



