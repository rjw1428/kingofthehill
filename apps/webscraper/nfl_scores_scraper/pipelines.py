from nfl_scores_scraper.items import NFLScoreGameItem
import sys
import pymongo

class NflScoresScraperPipeline:
    collection = 'nfl_scores'
    teams_collection = 'teams'

    def __init__(self): #, mongodb_uri, mongodb_db
        self.mongodb_uri = 'mongodb://localhost:27017'
        self.mongodb_db = 'kingofthehill'
        if not self.mongodb_uri: sys.exit("You need to provide a Connection String.")

    # @classmethod
    # def from_crawler(cls, crawler):
    #     return cls(
    #         mongodb_uri=crawler.settings.get('MONGODB_URI'),
    #         mongodb_db=crawler.settings.get('MONGODB_DATABASE', 'items')
    #     )
    def open_spider(self, spider):
        self.client = pymongo.MongoClient(self.mongodb_uri)
        self.db = self.client[self.mongodb_db]
        # Delete data from same week & year
        self.db[self.collection].delete_many({'week': spider.week, 'year': spider.year})

    
    def process_item(self, item, spider):
        data = dict(NFLScoreGameItem(item))
        home = data.get('home')
        away = data.get('away')

        home_team_data = self.db[self.teams_collection].find({"name": home['name']})[0]
        home_update = dict(home)
        home_update.update({'id': home_team_data['_id']})

        away_team_data = self.db[self.teams_collection].find({"name": away['name']})[0]
        away_update = dict(away)
        away_update.update({'id': away_team_data['_id']})

        winner = 'none'
        if int(home['score']) > int(away['score']):
            winner = 'home'
        elif  int(home['score']) < int(away['score']):
            winner = 'away'

        data = dict({
            'home': home_update,
            'away': away_update,
            'winner': winner,
            'week': spider.week, 
            'year': spider.year, 
            'winner': winner
        })
        self.db[self.collection].insert_one(data)
        return item

    # def process_item(self, item, spider):
    #     return item
    
    def close_spider(self, spider):
        self.client.close()
