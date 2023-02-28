FROM ubuntu:18.04

WORKDIR /data
# option [file, table[sqlite, postgree]]

# get data from storage
#       - transit_file

# create optional use SQL (import transit file if data tabular to table with auto colomn detect)
#       - transit_table
#       - output_table

COPY top_songs.csv .

ENV DATA_FORMAT=FILE

# install dependecies

COPY requirements.txt .

RUN apt -y update

RUN apt -y install python3 python3-pip

RUN python3 -m pip install --no-cache-dir -r requirements.txt

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

COPY pipe.py .

RUN python3 pipe.py

## send file back to storage as temp_data format

ENTRYPOINT ["ls", "-l"]