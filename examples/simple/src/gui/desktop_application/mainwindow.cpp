#include "mainwindow.h"
#include "car/car_controller.h"
#include "passenger/passenger_controller.h"
#include "single_passenger.h"
#include "ui_mainwindow.h"

#include <QCoroTimer>
#include <QProperty>
#include <QStyledItemDelegate>

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent), ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    // cars

    m_carModel = new Simple::Presenter::CarListModel(this);
    ui->carListView->setModel(m_carModel);

    connect(ui->addCarPushButton, &QPushButton::clicked, this, []() {
        auto *carController = Simple::Controller::Car::CarController::instance();
        auto create_DTO = carController->getCreateDTO();
        create_DTO.setContent("Example car %1"_L1.arg(QString::number(QDateTime::currentMSecsSinceEpoch())));

        Simple::Controller::Car::CarController::instance()->create(create_DTO);
    });

    connect(ui->removeCarPushButton, &QPushButton::clicked, this, [this]() {
        auto *carController = Simple::Controller::Car::CarController::instance();
        if (!ui->carListView->currentIndex().isValid())
            return;
        auto id = ui->carListView->currentIndex().data(Simple::Presenter::CarListModel::IdRole).toInt();
        carController->remove(id);
    });

    // passengers
    m_passengerModelFromCarPassengers = new Simple::Presenter::PassengerListModelFromCarPassengers(this);
    ui->passengerListView->setModel(m_passengerModelFromCarPassengers);

    connect(ui->addPassengerPushButton, &QPushButton::clicked, this, []() {
        auto *passengerController = Simple::Controller::Passenger::PassengerController::instance();
        auto create_DTO = passengerController->getCreateDTO();
        create_DTO.setName("Example passenger %1"_L1.arg(QString::number(QDateTime::currentMSecsSinceEpoch())));
        create_DTO.setCarId(1);
        create_DTO.setPosition(-1);

        Simple::Controller::Passenger::PassengerController::instance()->create(create_DTO);
    });

    // remove on double clicking on passengerListView
    connect(ui->passengerListView, &QListView::doubleClicked, [this](const QModelIndex &index) {
        if (!index.isValid())
            return;
        auto id = index.data(Simple::Presenter::PassengerListModelFromCarPassengers::IdRole).toInt();
        Simple::Controller::Passenger::PassengerController::instance()->remove(id);
    });

    // one passenger details

    auto *singlePassenger = new Simple::Presenter::SinglePassenger(this);
    connect(ui->passengerListView->selectionModel(), &QItemSelectionModel::currentChanged, singlePassenger,
            [singlePassenger](const QModelIndex &current, const QModelIndex &previous) {
                if (current.isValid())
                    singlePassenger->setId(
                        current.data(Simple::Presenter::PassengerListModelFromCarPassengers::IdRole).toInt());
                else
                    singlePassenger->setId(0);
            });

    // id's QSpinBox
    connect(singlePassenger, &Simple::Presenter::SinglePassenger::idChanged, ui->idSpinBox,
            [this, singlePassenger]() { ui->idSpinBox->setValue(singlePassenger->id()); });

    connect(ui->idSpinBox, &QSpinBox::valueChanged, singlePassenger,
            [this, singlePassenger](int value) { singlePassenger->setId(value); });

    // name's QLineEdit
    connect(singlePassenger, &Simple::Presenter::SinglePassenger::nameChanged, ui->nameLineEdit,
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
    createCarDTO.setContent("Example car 1"_L1);
    const auto &carDto = co_await carController->create(createCarDTO);

    auto *passengerController = Simple::Controller::Passenger::PassengerController::instance();
    auto create_DTO = passengerController->getCreateDTO();
    create_DTO.setName("Example passenger 1"_L1);
    create_DTO.setCarId(1);
    const auto &passengerDto = co_await passengerController->create(create_DTO);

    m_passengerModelFromCarPassengers->setCarId(carDto.id());
}
