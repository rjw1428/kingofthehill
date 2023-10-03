# Define here the models for your scraped items
#
# See documentation in:
# https://docs.scrapy.org/en/latest/topics/items.html

import scrapy


class NFLScoreTeamItem(scrapy.Item):
    name = scrapy.Field()
    score = scrapy.Field()
    pass

class NFLScoreGameItem(scrapy.Item):
    home = scrapy.Field(serializer=NFLScoreTeamItem) 
    away = scrapy.Field(serializer=NFLScoreTeamItem)
