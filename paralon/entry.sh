#!/bin/bash

./puller pull

python3 pipe.py

./puller push

ls -l