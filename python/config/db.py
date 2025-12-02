import pymongo
def connect_mongodb():
    client = pymongo.MongoClient("mongodb://127.0.0.1:27017")
    return client["Mini-CC"]