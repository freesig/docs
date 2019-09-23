#!/bin/bash
FILES=coreconcepts/*
for f in $FILES
do
  single_source md $f docs/$f
done
