FROM alpine:3.14

WORKDIR /data
# option [file, table[sqlite, postgree]]

# get data from storage
#       - transit_file

# create optional use SQL (import transit file if data tabular to table with auto colomn detect)
#       - transit_table
#       - output_table

COPY country_polution.csv .

ENV DATA_FORMAT=FILE

# install dependecies
ENV PYTHONUNBUFFERED=1
RUN apk add --update --no-cache python3 && ln -sf python3 /usr/bin/python
RUN python3 -m ensurepip
RUN pip3 install --no-cache --upgrade pip setuptools

COPY deps.txt .

RUN pip install -r deps.txt

# run some process to data ====start

# p = paralon.useSql("transit_table") # transform file to sql table
# p.Exec("UPDATE transit_table SET qualified=0 WHERE year > 2020;")
# my_data = p.Query("SELECT * FROM transit_table")
# p.Done()

# p = paralon.useFile()
# my_data = p.Get()
# some war process / manual
# p.Done()

# run some process to data ====end


ENTRYPOINT ["sleep", "500000"]