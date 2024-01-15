<!DOCTYPE html>
<html lang="tr">
	<head>
		<meta charset="utf-8" />
		<title></title>
		<meta name="viewport" content="initial-scale=1,maximum-scale=1,user-scalable=no" />
		<link href="https://api.mapbox.com/mapbox-gl-js/v3.0.1/mapbox-gl.css" rel="stylesheet" />
		<script src="https://api.mapbox.com/mapbox-gl-js/v3.0.1/mapbox-gl.js"></script>
		<link
			rel="stylesheet"
			href="https://fonts.googleapis.com/css2?family=Poppins:wght@400;700&display=swap"
		/>
		<link rel="stylesheet" href="style.css" />
		<script
			src="https://cdn.jsdelivr.net/gh/jscastro76/threebox@v.2.2.7/dist/threebox.min.js"
			type="text/javascript"
		></script>
		<link
			href="https://cdn.jsdelivr.net/gh/jscastro76/threebox@v.2.2.7/dist/threebox.css"
			rel="stylesheet"
		/>
	</head>

	<body>
		<div id="map-container">
			<div class="fixed top-0 left-0 w-full bg-gray-800 p-4 z-10 flex justify-between items-center">
				<div class="flex items-center">
					<svg
						class="h-6 w-6 text-white cursor-pointer mr-4 hamburger-icon"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M4 6h16M4 12h16m-7 6h7"
						></path>
					</svg>
					<h1 class="text-white text-lg font-bold">Yavuz</h1>
				</div>
				<button
					id="flyToUAVButton"
					class="text-white bg-blue-500 px-4 py-2 rounded hover:bg-blue-600 focus:outline-none navbar-button"
				>
					İHA Pozisyonuna git
				</button>
				<img src="uav.jpg" alt="Navbar Icon" class="ml-2" width="30" height="30" />
			</div>

			<div id="map"></div>
			<!-- <pre id="info" class="z-40"></pre> -->

			<div
				id="marks-list"
				class="bg-gray-800 text-white p-4 fixed top-0 z-30 right-0 h-full overflow-auto hidden"
			>
				<h2 class="text-lg font-semibold mb-2">Lokasyon Listesi</h2>
				<ul id="marks-list-ul"></ul>
			</div>
		</div>

		<div id="modal-container">
			<div id="modal-content"></div>
		</div>

		<script>
			let firstPos;

			mapboxgl.accessToken =
				'pk.eyJ1IjoidGVzdGFjY3VhdiIsImEiOiJjbHE1OTh3OGMwYW5mMmlrejQzbjdseGNxIn0.0f4Ih15YAO4lvdXfTawzZg';
			const map = new mapboxgl.Map({
				container: 'map',
				style: 'mapbox://styles/mapbox/streets-v12',
				zoom: 18,
				center: firstPos,
				pitch: 60,
				antialias: true
			});

			const tb = (window.tb = new Threebox(map, map.getCanvas().getContext('webgl'), {
				defaultLights: true
			}));

			let devmode;

			setTimeout(async () => {
				const uavCoordinates = await fetchUAVCoordinates();
				map.flyTo({
					center: uavCoordinates,
					zoom: 18,
					essential: true,
					duration: 1000
				});
			}, 1000);

			let uavMarker;

			map.on('style.load', () => {
				map.addLayer({
					id: 'custom-threebox-model',
					type: 'custom',
					renderingMode: '3d',
					onAdd: function () {
						const scale = 0.5;
						const options = {
							obj: 'yavuz.gltf',
							type: 'gltf',
							scale: { x: scale, y: scale, z: 0.5 },
							units: 'meters',
							rotation: { x: 90, y: -90, z: 0 },
							adjustment: { x: -0.9, y: 0.499, z: -0.5 }
						};

						tb.loadObj(options, async (model) => {
							const initialCoords = await fetchUAVCoordinates();

							const el = document.createElement('div');
							el.className = 'uav-marker';
							uavMarker = new mapboxgl.Marker(el).setLngLat(initialCoords).addTo(map);

							uavMarker = el;

							uavMarker.addEventListener('click', () => {
								map.flyTo({
									center: initialCoords,
									zoom: 18,
									essential: true
								});
							});

							model.setCoords(initialCoords);
							model.setRotation({ x: 90, y: 0, z: 270 });
							tb.add(model);
							setInterval(async () => {
								const updatedCoords = await fetchUAVCoordinates();
								console.log('updatedCoords: ', updatedCoords);
								uavMarker.remove();
								const el2 = document.createElement('div');
								el.className = 'uav-marker';
								uavMarker = new mapboxgl.Marker(el).setLngLat(updatedCoords).addTo(map);
								uavMarker = el2;
								uavMarker.addEventListener('click', () => {
									map.flyTo({
										center: initialCoords,
										zoom: 18,
										essential: true
									});
								});
								model.setCoords(updatedCoords);
							}, 5000);
						});
					},

					render: function () {
						tb.update();
					}
				});
			});

			async function addMarkers() {
				try {
					const response = await fetch('http://yavuz.tahakocabuga.com/api/marks');
					const marks = await response.json();

					marks.forEach((mark) => {
						const el = document.createElement('div');
						el.className = 'marker';

						const marker = new mapboxgl.Marker(el)
							.setLngLat([mark.latitude, mark.longitude])
							.addTo(map);

						el.addEventListener('click', () => {
							if (map.getZoom() < 16) {
								map.flyTo({
									center: [mark.latitude, mark.longitude],
									zoom: 17,
									essential: true
								});
							} else {
								console.log('Marker clicked:', mark);
								const imagesArray = JSON.parse(mark.images);
								let trimmedString = imagesArray.slice(1, -1);
								let arr = trimmedString.split(', ');

								const popupContent = `
									<div class="bg-white p-4 rounded shadow-lg">
										<p class="text-sm mb-1 text-gray-500">ID: ${mark.id}</p>
										<h2 class="text-xl mb-2 font-bold"> ${mark.title}</h2>
										<p class="text-sm mb-1">Zaman: ${mark.time}</p>
										<p class="text-sm mb-2">Kordinatlar: [${mark.latitude}, ${mark.longitude}]</p>
										
										<div class="mt-2 grid grid-cols-3 gap-1 popup-images">
										${arr
											.map(
												(image) =>
													`<img src="${image}" alt="Image" class="rounded popup-image cursor-pointer" onclick="openModal('${image}')">`
											)
											.join('')}
										</div>
									</div>
								`;

								new mapboxgl.Popup({ closeOnClick: false })
									.setLngLat([mark.latitude, mark.longitude])
									.setMaxWidth('400px')
									.setHTML(popupContent)
									.addTo(map);
							}
						});

						const marksListUl = document.getElementById('marks-list-ul');
						const listItem = document.createElement('li');
						listItem.className = 'cursor-pointer p-2 hover:bg-gray-900 mb-2 rounded';
						listItem.innerHTML = `
            <div>
  <h3 class="text-lg font-semibold mb-1">${mark.title}</h3>
  <p class="text-sm mb-1 text-gray-400">Zaman: ${mark.time}</p>
  <p class="text-sm mb-1 text-gray-400">Kordinatlar: [${mark.latitude}, ${mark.longitude}]</p>
  <p class="text-sm mb-1 text-gray-400">ID: ${mark.id}</p>
  <button class="bg-red-500 text-white px-2 py-1 rounded mt-1" onclick="deleteMark(${mark.id}, event)">
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-trash-2"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
  </button>
  <button class="bg-green-500 text-white px-2 py-1 rounded mt-1" onclick="copyCoordinates(${mark.latitude}, ${mark.longitude})">
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-clipboard"><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path><rect x="8" y="2" width="8" height="4" rx="1" ry="1"></rect></svg>
  </button>
</div>
            `;
						listItem.addEventListener('click', () => {
							map.flyTo({
								center: [mark.latitude, mark.longitude],
								zoom: 18,
								essential: true
							});
						});
						marksListUl.appendChild(listItem);
					});
				} catch (error) {
					console.error('Error fetching /marks:', error);
				}
			}

			function openModal(imageSrc) {
				const modalContainer = document.getElementById('modal-container');
				const modalContent = document.getElementById('modal-content');

				modalContent.innerHTML = `
          <span class="modal-close" onclick="closeModal()">&times;</span>
          <img src="${imageSrc}" style="width: 100%; height: 100%;" />
        `;

				modalContainer.style.display = 'flex';
			}

			function closeModal() {
				const modalContainer = document.getElementById('modal-container');
				modalContainer.style.display = 'none';
			}

			function toggleMenu() {
				const marksList = document.getElementById('marks-list');
				marksList.classList.toggle('hidden');
			}

			const hamburgerIcon = document.querySelector('.hamburger-icon');
			if (hamburgerIcon) {
				hamburgerIcon.addEventListener('click', toggleMenu);
			}

			map.on('load', function () {
				addMarkers();
			});

			async function deleteMark(id) {
				try {
					event.stopPropagation();
					const response = await fetch(`http://yavuz.tahakocabuga.com/api/marks/${id}`, {
						method: 'DELETE'
					});

					if (response.ok) {
						console.log(`Mark with ID ${id} deleted successfully`);
						await reloadMarkers();
					} else {
						console.error(`Error deleting mark with ID ${id}`);
					}
				} catch (error) {
					console.error('Error deleting mark:', error);
				}
			}

			function clearMarkers() {
				const markers = document.querySelectorAll('.marker');
				markers.forEach((marker) => marker.remove());
			}

			async function reloadMarkers() {
				clearMarkers();

				const marksListUl = document.getElementById('marks-list-ul');
				marksListUl.innerHTML = '';

				await addMarkers();
			}

			function copyCoordinates(latitude, longitude) {
				navigator.clipboard.writeText(`${latitude}, ${longitude}`);
			}

			setInterval(reloadMarkers, 20000);

			async function fetchUAVCoordinates() {
				const response = await fetch('http://yavuz.tahakocabuga.com/api/uav');
				const { latitude, longitude } = await response.json();
				return [latitude, longitude];
			}

			const flyToUAVButton = document.getElementById('flyToUAVButton');
			if (flyToUAVButton) {
				flyToUAVButton.addEventListener('click', async () => {
					try {
						const uavCoordinates = await fetchUAVCoordinates();
						map.flyTo({
							center: uavCoordinates,
							zoom: 18,
							essential: true
						});
					} catch (error) {
						console.error('Error fetching UAV coordinates:', error);
					}
				});
			}

			// map.on('mousemove', (e) => {

			// 		document.getElementById('info').innerHTML =
			// 			JSON.stringify(e.point) + '<br />' + JSON.stringify(e.lngLat.wrap());

			// });
		</script>
	</body>
</html>
