from flask import Flask, Response, request

app = Flask(__name__)


@app.route('/k2/public/api/1/carddata')
def card_data():
    resp_type = request.args.get('resp_type')
    file_to_load = 'tests/example_response.json'

    if resp_type == "with_many_nulls":
        file_to_load = 'tests/example_response_with_many_nulls.json'

    with open(file_to_load, 'r') as example_resp:
        return Response(example_resp.read(), mimetype="application/json")


if __name__ == '__main__':
    app.run()
