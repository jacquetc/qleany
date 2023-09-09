#include "mainwindow.h"
#include "car/car_controller.h"
#include "passenger/passenger_controller.h"
#include "single_passenger.h"
#include "ui_mainwindow.h"

#include <QCoroTask>
#include <QCoroTimer>
#include <QProperty>

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent), ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    m_passengerModel = new PassengerListModel(this);
    ui->passengerListView->setModel(m_passengerModel);

    connect(ui->addPassengerPushButton, &QPushButton::clicked, []() {
        auto *passengerController = Simple::Controller::Passenger::PassengerController::instance();
        auto create_DTO = passengerController->getCreateDTO();
        create_DTO.setName("Example passenger");

        Simple::Controller::Passenger::PassengerController::instance()->create(create_DTO);
    });

    auto *singlePassenger = new SinglePassenger(this);
    connect(ui->passengerListView->selectionModel(), &QItemSelectionModel::currentChanged, singlePassenger,
            [singlePassenger](const QModelIndex &current, const QModelIndex &previous) {
                if (!current.isValid())
                    return;
                singlePassenger->setId(current.data(PassengerListModel::Id).toInt());
            });

    // id's QSpinBox
    connect(singlePassenger, &SinglePassenger::idChanged, ui->idSpinBox,
            [this, singlePassenger]() { ui->idSpinBox->setValue(singlePassenger->id()); });

    connect(ui->idSpinBox, &QSpinBox::valueChanged, singlePassenger,
            [this, singlePassenger](int value) { singlePassenger->setId(value); });

    // name's QLineEdit
    connect(singlePassenger, &SinglePassenger::nameChanged, ui->nameLineEdit,
            [this, singlePassenger]() { ui->nameLineEdit->setText(singlePassenger->name()); });

    connect(ui->nameLineEdit, &QLineEdit::editingFinished, singlePassenger, [this, singlePassenger]() {
        QString text = ui->nameLineEdit->text();
        singlePassenger->setName(text);
    });

    QTimer::singleShot(0, this, [this]() -> QCoro::Task<> { // add dummy
        auto *carController = Simple::Controller::Car::CarController::instance();
        auto createCarDTO = carController->getCreateDTO();
        createCarDTO.setContent("Example car 1");
        const auto &carDto = co_await carController->create(createCarDTO);

        auto *passengerController = Simple::Controller::Passenger::PassengerController::instance();
        auto create_DTO = passengerController->getCreateDTO();
        create_DTO.setName("Example passenger 1");
        create_DTO.setCarId(1);
        const auto &passengerDto = co_await passengerController->create(create_DTO);

        m_passengerModel->setCarId(carDto.id());

    });
}

MainWindow::~MainWindow()
{
    delete ui;
}
