export class LineChart {
    chart;

    constructor(labels) {        
        let data = {
            labels: labels,
            datasets: [{
                label: 'Widget data',
                backgroundColor: 'rgb(255, 99, 132)',
                borderColor: 'rgb(255, 99, 132)',
                data: [10,35,30,20,25,15],
            }]
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
}
