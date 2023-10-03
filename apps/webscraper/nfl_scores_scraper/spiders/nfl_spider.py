from nfl_scores_scraper.items import NFLScoreGameItem
from nfl_scores_scraper.items import NFLScoreTeamItem
import scrapy


class NFLSpider(scrapy.Spider):
    name = "nflspider"
    allowed_domains = ["espn.com"]

    def __init__(self, week='', year='', **kwargs):
        self.year = year
        self.week = week
        self.start_urls = [f'https://espn.com/nfl/scoreboard/_/week/{week}/year/{year}/seasontype/2']
        super().__init__(**kwargs)

    def parse(self, response):
        for score_elem in response.css('.ScoreboardScoreCell'):
            team = score_elem.css('.ScoreCell__TeamName::text').getall()
            score = score_elem.css('.ScoreCell__Score::text').getall()
            if (len(score) == 0):
                score = [0, 0]
            print(f'HOME: {team[1]} - {score[1]} AWAY: {team[0]} - {score[0]}')
            home = NFLScoreTeamItem(name=team[1], score=score[1])
            away = NFLScoreTeamItem(name=team[0], score=score[0])
            game = NFLScoreGameItem(home=home, away=away)
            yield game