document.querySelectorAll(".button").forEach((button) => {
  button.addEventListener("click", function () {
    const type = this.dataset.type;

    fetch("/api/tasks", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ type }),
    })
      .then((res) => res.json())
      .then((data) => {
        getStatus(data.task_id);
      })
      .catch((err) => {
        console.log(err);
      });
  });
});

function getStatus(taskID) {
  fetch(`/api/tasks/${taskID}`, {
    method: "GET",
  })
    .then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error: ${response.status}`);
      }
      return response.json();
    })
    .then((res) => {
      const html = `
        <tr>
          <td>${res.task_id}</td>
          <td>${res.task_status}</td>
          <td>${res.task_result}</td>
        </tr>
      `;

      document.querySelector("#tasks").insertAdjacentHTML("afterbegin", html);

      const taskStatus = res.task_status;

      if (taskStatus === "SUCCESS" || taskStatus === "FAILURE") return;

      setTimeout(() => {
        getStatus(res.task_id);
      }, 1000);
    })
    .catch((err) => {
      console.log(err);
    });
}
