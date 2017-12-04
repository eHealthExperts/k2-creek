from flask import Flask, Response

app = Flask(__name__)


@app.route('/k2/public/api/1/carddata')
def home():
    with open('tests/example_response.json', 'r') as example_resp:
        return Response(example_resp.read(), mimetype="application/json")


if __name__ == '__main__':
    app.run()
