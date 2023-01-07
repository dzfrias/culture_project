export class LineChart {
    chart;

    constructor(labels, datasets) {        
        let data = {
            labels: labels,
            datasets: datasets
        };

        this.config = {
            type: 'line',
            data: data,
            options: {
                responsive: false,
                scales: {
                    y: {
                        suggestedMin: 0,
                        suggestedMax: 50
                    }
                }
            }
        };
    }

    draw(element_id) {
        this.chart = new Chart(
            document.getElementById(element_id),
            this.config
        )
    }

    new_config(labels, datasets) {
        let data = {
            labels: labels,
            datasets: datasets
        };

        this.config = {
            type: 'line',
            data: data,
            options: {
                responsive: false,
                scales: {
                    y: {
                        suggestedMin: 0,
                        suggestedMax: 50
                    }
                }
            }
        };
        this.chart.data = data;
        this.chart.update();
    }
}
