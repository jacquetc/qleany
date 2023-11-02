#include "mainwindow.h"
#include "car/car_controller.h"
#include "passenger/passenger_controller.h"
#include "single_passenger.h"
#include "ui_mainwindow.h"

#include <QCoroTimer>
#include <QProperty>

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent), ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    m_passengerModel = new PassengerListModel(this);
    ui->passengerListView->setModel(m_passengerModel);

    connect(ui->addPassengerPushButton, &QPushButton::clicked, this, []() {
        auto *passengerController = Simple::Controller::Passenger::PassengerController::instance();
        auto create_DTO = passengerController->getCreateDTO();
        create_DTO.setName(QString("Example passenger %1").arg(QDateTime::currentMSecsSinceEpoch()));
        create_DTO.setCarId(1);
        create_DTO.setPosition(-1);

        Simple::Controller::Passenger::PassengerController::instance()->create(create_DTO);
    });

    auto *singlePassenger = new SinglePassenger(this);
    connect(ui->passengerListView->selectionModel(), &QItemSelectionModel::currentChanged, singlePassenger,
            [singlePassenger](const QModelIndex &current, const QModelIndex &previous) {
                if (!current.isValid())
                    return;
                singlePassenger->setId(current.data(PassengerListModel::Id).toInt());
            });

    // remove on double clicking on passengerListView
    connect(ui->passengerListView, &QListView::doubleClicked, [this](const QModelIndex &index) {
        if (!index.isValid())
            return;
        auto id = index.data(PassengerListModel::Id).toInt();
        Simple::Controller::Passenger::PassengerController::instance()->remove(id);
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
}

MainWindow::~MainWindow()
{
    delete ui;
}

QCoro::Task<> MainWindow::init()
{
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
}
