function map_to_object(map) {
    const out = Object.create(null);
    map.forEach((value, key) => {
      if (value instanceof Map) {
        out[key] = map_to_object(value);
      } else {
        out[key] = value;
      }
    });
    return out;
}

export class LineChart {
    chart;

    constructor(labels, datasets, opts) {        
        let data = {
            labels: labels,
            datasets: datasets
        };

        this.config = {
            type: 'line',
            data: data,
            options: map_to_object(opts),
        };
    }

    draw(element_id) {
        this.chart = new Chart(
            document.getElementById(element_id),
            this.config
        )
    }

    newConfig(labels, datasets, opts) {
        let data = {
            labels: labels,
            datasets: datasets
        };

        this.chart.data = data;
        this.chart.options = map_to_object(opts);
        this.chart.update();
    }
}
