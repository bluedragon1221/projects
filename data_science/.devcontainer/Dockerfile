FROM frolvlad/alpine-miniconda3

RUN apk add curl

RUN conda install --yes \
    ipykernel IPython \
    numpy pandas \
    matplotlib seaborn plotly \
    pydantic