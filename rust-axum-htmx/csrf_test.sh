#!/bin/bash

# should returm html with "Wrong csrf token..."
curl -X POST localhost:4000/form \
   -H "Content-Type: application/x-www-form-urlencoded" \
   -d "data=csrf1!!&csrf_token=hjadfsjhiasdf" 
