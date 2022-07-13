import json
import random
import string

from locust import HttpUser, task


class ProjectUsage(HttpUser):
    @task
    def get_html(self):
        self.client.get("/")

    @task
    def get_daily_pic(self):
        self.client.get("/api/daily_photo")

    @task
    def get_todos(self):
        self.client.get("/api/todos")

    @task
    def add_todo(self):
        letters = string.ascii_lowercase
        random_task = "".join(random.choice(letters) for i in range(10))
        headers = {
            "Content-Type": "application/json"
        }
        self.client.post("/api/todo", json.dumps({"task": random_task}), headers=headers)
